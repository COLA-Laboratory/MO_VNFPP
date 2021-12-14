addpath(".");

close all
clear
clc
format long g

% root_path = "/media/joebillingsley/Data/projects/NFV_PlacementModel_Journal";
root_path = "D:\Research\NFV_PlacementModel_Journal";

src_folder = fullfile(root_path, "processed", "aggregate", "AntiAffinity");
out_folder = fullfile(root_path, "processed", "anti_affinity");

if not(isfolder(out_folder))
    mkdir(out_folder)
end

perc_isolated = ["10", "20", "30", "40", "50", "60", "70", "80", "90", "100",];
algs = ["NSGAII"];
algs_out = ["NSGA-II"];

ops = ["std","ca"];

for i = 1:length(algs)
    
    out_file = fullfile(out_folder, strcat(algs, ".csv"));
    
    fid = fopen(out_file, 'w');
    fprintf(fid, "k,std_lb,std_median,std_ub,ca_lb,ca_median,ca_ub\n");
    
    for j = 1:length(perc_isolated)
        pi = perc_isolated(j);
        
        alg = algs(i);
        alg_print = algs_out(i);
        
        fprintf(fid, '%s,', pi);
        
        for k = 1:length(ops)
            op = ops(k);
            
            hv_file = fullfile(src_folder, pi, op, alg, "aggregate.csv");
            agg = csvread(hv_file);
            median = agg(6);
            lb = median - agg(5);
            ub = agg(7) - median;
            
            fprintf(fid, '%f,%f,%f', lb, median, ub);
            
            if k < length(ops)
                fprintf(fid, ',');
            end
        end
        
        fprintf(fid, '\n');
    end
end

fclose(fid);