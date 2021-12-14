items = dir('/media/joebillingsley/Data/projects/NFV_PlacementModel_Journal/data/**/*.objs');

for i = 1:length(items)
    file = items(i);
    path = fullfile(file.folder, file.name);
    
    % Read the file
    fid = fopen(path,'r');
    pop = textscan(fid,'%s','Delimiter','\n');
    fclose(fid);
    
    % Extract first 120 lines
    fixed_pop = pop{1}(1:min(120, length(pop{1})));
    
    % Save as a text file
    fid = fopen(path,'w');
    fprintf(fid,'%s\n', fixed_pop{:});
    fclose(fid);
end